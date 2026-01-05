extends Node

class_name Board

@export var tetromino_scene: PackedScene
var tetrominos: Array[Tetromino] = []
signal tetromino_locked


func spawn_tetromino(type: Shared.Tetromino, is_next_piece, spawn_position):
	var tetromino_data: PieceData = Shared.data[type]

	var tetromino = tetromino_scene.instantiate().init(tetromino_data, is_next_piece) as Tetromino

	if is_next_piece == false:
		tetromino.position = tetromino_data.spawn_position
		tetromino.other_tetrominos = tetrominos
		tetromino.lock_tetromino.connect(on_tetromino_locked)
		add_child(tetromino)


func on_tetromino_locked(tetromino: Tetromino):
	tetrominos.append(tetromino)
	tetromino_locked.emit()
