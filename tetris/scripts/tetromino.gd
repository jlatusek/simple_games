extends Node2D

class_name Tetromino

var tetromino_data: PieceData
var is_next_piece
var pieces: Array[Piece] = []
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


func _input(_event: InputEvent) -> void:
	if Input.is_action_just_pressed("left"):
		move(Vector2.LEFT)
	elif Input.is_action_just_pressed("right"):
		move(Vector2.RIGHT)
	elif Input.is_action_just_pressed("down"):
		move(Vector2.DOWN)
	elif Input.is_action_just_pressed("hard_drop"):
		pass
	elif Input.is_action_just_pressed("rotate_left"):
		pass
	elif Input.is_action_just_pressed("rotate_right"):
		pass


func move(direction: Vector2) -> bool:
	var new_position = calculate_global_position(direction, global_position)
	if new_position:
		global_position = new_position
		return true
	return false


func calculate_global_position(direction: Vector2, starting_global_position: Vector2) -> Vector2:
	return starting_global_position + direction * pieces[0].get_size()


func _on_timer_timeout() -> void:
	move(Vector2.DOWN)
