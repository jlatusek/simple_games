extends Node

@onready var current_tetromino: Shared.Tetromino = Shared.Tetromino.values().pick_random()

@onready var board = $"../Board" as Board


func _ready() -> void:
	board.spawn_tetromino(current_tetromino, false, null)
