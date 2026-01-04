extends Node2D

class_name Tetromino

var tetromino_data: PieceData
var is_next_piece
var pieces = []
var wall_kicks

@onready var tetromino_cells = Shared.cells[tetromino_data.tetromino_type]
@onready var piece_scene = preload("res://scenes/piece.tscn")


func _ready() -> void:
	for cell in tetromino_cells:
		var piece = piece_scene.instantiate() as Piece
		pieces.append(piece)
		add_child(piece)
		piece.set_texture(tetromino_data.piece_texture)
		piece.position = cell * piece.get_size()

	if is_next_piece == false:
		position = tetromino_data.spawn_position
		wall_kicks = (
			Shared.wall_kicks_i
			if tetromino_data.tetromino_type == Shared.Tetromino.I
			else Shared.wall_kicks_jlostz
		)


func init(tetromino_data: PieceData, is_next_piece: bool) -> Tetromino:
	self.tetromino_data = tetromino_data
	self.is_next_piece = is_next_piece
	return self
