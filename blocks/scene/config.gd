extends Node2D

const FALL_SPEED = 100
const BLOCK_SIZE = 40
const SCENE_WIDTH = 480
const SCENE_HIGHT = 720

@warning_ignore("integer_division")
const SCENE_BLOCK_HIGHT = int(SCENE_HIGHT / BLOCK_SIZE)
@warning_ignore("integer_division")
const SCENE_BLOCK_WIDTH = int(SCENE_WIDTH / BLOCK_SIZE)

const PLAYAREA_BLOCK_HIGHT = SCENE_BLOCK_HIGHT - 1
const PLAYAREA_BLOCK_WIDTH = SCENE_BLOCK_WIDTH - 2
