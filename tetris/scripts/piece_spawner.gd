extends Node

@onready var current_tetromino: Shared.Tetromino = Shared.Tetromino.values().pick_random()
@onready var board = $"../Board" as Board


func _ready() -> void:
	board.spawn_tetromino(current_tetromino, false, null)
	board.tetromino_locked.connect(on_tetromino_locked)


func on_tetromino_locked():
	var new_tetromino = Shared.Tetromino.values().pick_random()
	board.spawn_tetromino(new_tetromino, false, null)
