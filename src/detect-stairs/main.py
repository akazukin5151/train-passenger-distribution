from typing import Tuple
import sys
import subprocess
import io
from pathlib import Path
import cv2
import numpy as np
import numpy.typing as npt
import pandas as pd
#import matplotlib.pyplot as plt

Bounds = Tuple[int, int, int, int]

def bbox(mask: npt.NDArray, buffer: int) -> Bounds:
    rows = np.any(mask, axis=1)
    cols = np.any(mask, axis=0)
    rmin, rmax = np.where(rows)[0][[0, -1]]
    cmin, cmax = np.where(cols)[0][[0, -1]]
    return (rmin - buffer, rmax + buffer, cmin - buffer, cmax + buffer)

def setup() -> Tuple[npt.NDArray, int, int]:
    # with a low enough threshold, this is good enough to match the other 3 as well
    escalator_left = cv2.imread('src/detect-stairs/escalator_left.png')
    w, h = escalator_left.shape[:-1]
    return (escalator_left, w, h)

def find_platforms(station_layout: npt.NDArray) -> npt.NDArray:
    platform_color_1 = (182, 220, 241)
    platform_color_2 = (185, 227, 249)

    platform_color_corrected = cv2.cvtColor(station_layout, cv2.COLOR_BGR2RGB)
    return cv2.inRange(platform_color_corrected, platform_color_1, platform_color_2)

def find_line_row(
    platforms: npt.NDArray,
    platforms_mask: npt.NDArray,
    rmin: int,
    rmax: int
) -> Tuple[npt.NDArray, Bounds]:
    line_color = (235, 92, 2)
    # TODO: need to color correct it again for some reason
    platform_color_corrected = cv2.cvtColor(platforms, cv2.COLOR_BGR2RGB)
    mask = cv2.inRange(platform_color_corrected, line_color, line_color)
    (rmin_, rmax_, cmin, cmax) = bbox(mask, 0)

    max_row_idx = rmin + rmax_
    while np.any(platforms_mask[max_row_idx]):
        max_row_idx += 1

    min_row_idx = rmin + rmin_
    while np.any(platforms_mask[min_row_idx]):
        min_row_idx -= 1
    return (mask, (min_row_idx, max_row_idx, cmin, cmax))

def find_stairs(
    platform: npt.NDArray, escalator_left: npt.NDArray, w: int, h: int
) -> npt.NDArray:
    threshold = .35
    res = cv2.matchTemplate(platform, escalator_left, cv2.TM_CCOEFF_NORMED)
    loc = np.where(res >= threshold)
    for pt in zip(*loc[::-1]):  # Switch columns and rows
        cv2.rectangle(platform, pt, (pt[0] + w, pt[1] + h), (0, 0, 255), 2)

    return platform

def find_platform_cols(
    platforms_mask: npt.NDArray,
    line_min_row: int,
    line_max_row: int
) -> Tuple[int, int]:
    submask = platforms_mask[line_min_row:line_max_row, :]
    (_, _, cmin_, cmax_) = bbox(submask, 0)
    return (cmin_, cmax_)

#def match_features(cropped: npt.NDArray, escalator_left: npt.NDArray) -> None:
#    sift = cv2.SIFT_create()
#    kp1, des1 = sift.detectAndCompute(cropped, None)
#    kp2, des2 = sift.detectAndCompute(escalator_left, None)
#    FLANN_INDEX_KDTREE = 1
#    index_params = dict(algorithm=FLANN_INDEX_KDTREE, trees=5)
#    search_params = dict(checks=50)   # or pass empty dictionary
#    flann = cv2.FlannBasedMatcher(index_params, search_params)
#    matches = flann.knnMatch(des1, des2, k=2)
#    matchesMask = [[0, 0] for i in range(len(matches))]
#    for i, (m, n) in enumerate(matches):
#        if m.distance < 0.7 * n.distance:
#            matchesMask[i] = [1, 0]
#    draw_params = dict(matchColor=(0, 255, 0),
#                       singlePointColor=(255, 0, 0),
#                       matchesMask=matchesMask,
#                       flags=cv2.DrawMatchesFlags_DEFAULT)
#    img3 = cv2.drawMatchesKnn(
#        cropped, kp1, escalator_left, kp2, matches, None, **draw_params
#    )
#    plt.imshow(img3)
#    plt.show()
#    # TODO: don't know how to extract the points into squares...
#    # or maybe i just get the coordinate of the points?

def main(
    path: Path,
    query_text: str,
    escalator_left: npt.NDArray,
    w: int,
    h: int
) -> None:
    print(path)
    station_layout = cv2.imread(str(path))

    platforms_mask = find_platforms(station_layout)
    # bounds that includes all platforms, relative to `station_layout`
    (rmin, rmax, cmin, cmax) = bbox(platforms_mask, 50)
    all_platforms = station_layout[rmin:rmax, cmin:cmax]

    # row bounds for the line, relative to `station_layout`
    (line_color_mask, (line_min_row, line_max_row, mask_col, _)) = find_line_row(
        all_platforms,
        platforms_mask,
        rmin,
        rmax
    )
    # col bounds for the line's row_bounded_platform,
    # relative to `platforms_mask` (same as `station_layout`)
    (line_min_col, line_max_col) = find_platform_cols(
        platforms_mask,
        line_min_row,
        line_max_row
    )
    platform = station_layout[
        line_min_row:line_max_row, line_min_col:line_max_col
    ]

    # i tried using nonzero on line_color_mask to find the exact rows
    # of the text, so that ocr can operate only on the area of interest
    # but ocr was more unreliable and failed to read text with a line
    # going through it, while it could read it on the full platform image
    (_, buf) = cv2.imencode(path.name, platform)
    args = [
        sys.argv[1],
        'stdin',
        'stdout',
        '-l',
        'jpn',
        'tsv'
    ]
    p = subprocess.run(
        args,
        input=buf.tobytes(),
        # --tessdata-dir arg doesn't work for some reason
        env={'TESSDATA_PREFIX': './tessdata'},
        capture_output=True
    )
    tsv = p.stdout.decode('utf8')

    nonzeros = np.nonzero(line_color_mask[:, mask_col])[0]
    abs_diff = np.abs(np.diff(nonzeros))
    discontinunities_idx = np.where(abs_diff > 1)[0]
    discontinunities_idx_1 = discontinunities_idx + 1
    c = np.empty(
        (discontinunities_idx.size + discontinunities_idx_1.size,),
        dtype=discontinunities_idx.dtype
    )
    c[0::2] = discontinunities_idx
    c[1::2] = discontinunities_idx_1
    x = np.insert(c, 0, 0)
    x = np.insert(x, x.shape[0], -1)
    sections = np.split(nonzeros[x], x.shape[0] // 2)

    f = io.StringIO(tsv)
    df = pd.read_csv(f, sep='\t')
    df = df[['block_num', 'line_num', 'top', 'text']].dropna()
    df['text'] = df.text.str.strip()
    df = df[df['text'] != '']
    lines = df.groupby(['block_num', 'line_num']).agg({
        'top': 'mean',
        'text': 'sum'
    })
    # lol numpy doesn't have find first
    q = lines[lines.text.str.contains(query_text)].iloc[0]
    diff = line_min_row - rmin
    top = q['top'] + diff
    for [s_top, s_bottom] in sections:
        if s_top <= top <= s_bottom:
            break

    stairs = find_stairs(platform[s_top - diff:], escalator_left, w, h)
    cv2.imwrite('stairs/' + path.name, stairs)


(escalator_left, w, h) = setup()
for path in Path('maps/').iterdir():
    if path.suffix == '.png':
        query_text = '高尾' if path.name == '神田.png' else '立川'
        main(path, query_text, escalator_left, w, h)
#path_ = Path('maps/三鷹.png')
#main(path_)

