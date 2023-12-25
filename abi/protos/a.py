#计算两个框的 iou
def iou(a, b):
    # a: (x1, y1, x2, y2), b: (x1, y1, x2, y2)
    #计算两个框的面积
    area_a = (a[2] - a[0]) * (a[3] - a[1])
    area_b = (b[2] - b[0]) * (b[3] - b[1])
    #计算交集的坐标
    overlap_x1 = max(a[0], b[0])
    overlap_y1 = max(a[1], b[1])
    overlap_x2 = min(a[2], b[2])
    overlap_y2 = min(a[3], b[3])
    #计算交集的面积
    overlap_area = max(0, overlap_x2 - overlap_x1) * max(0, overlap_y2 - overlap_y1)
    #计算iou
    iou = overlap_area / (area_a + area_b - overlap_area)
    return iou

# 使用 diffusers 库写一个 stable diffusion 推理的例子
# Path: abi/protos/a.py
import diffusers
import numpy as np
import torch
import torchvision
from PIL import Image
from torchvision import transforms
pipeline = diffusers.Pipeline('stable_diffusion')
pipeline.to('cuda')
pipeline('a girl is playing tennis').image[0].save('a.png')