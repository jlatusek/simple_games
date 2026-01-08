extends Node2D

class_name GhostTetromino

@onready var piece_scene := preload("res://scenes/piece.tscn")
@onready var ghost_textire = preload("res://assets/Ghost.png")
var tetromino_data: PieceData


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var tetromino_cells = Shared.cells[tetromino_data.tetromino_type]

	for cell in tetromino_cells:
		var piece = piece_scene.instantiate()
		add_child(piece)
		piece.set_texture(ghost_textire)
		piece.position = cell * piece.get_size()


func set_ghost_tetromino(new_position: Vector2, pieces_position):
	global_position = new_position

	var pieces = get_children()
	for i in pieces.size():
		pieces[i].position = pieces_position[i]
