extends Node

class_name Board

@export var tetromino_scene: PackedScene


func spawn_tetromino(type: Shared.Tetromino, is_next_piece, spawn_position):
	var tetromino_data: PieceData = Shared.data[type]

	var tetromino = tetromino_scene.instantiate().init(tetromino_data, is_next_piece)

	if is_next_piece == false:
		tetromino.position = tetromino_data.spawn_position
		add_child(tetromino)
