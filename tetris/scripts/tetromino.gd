extends Node2D

class_name Tetromino
signal lock_tetromino(tetromino: Tetromino)


class Bounds:
	var x: Vector2
	var y: Vector2

	func _init(x: Vector2, y: Vector2) -> void:
		self.x = x
		self.y = y


@onready var bounds = Bounds.new(Vector2(-216, 216), Vector2(0, 457))

var tetromino_data: PieceData
var is_next_piece
var pieces: Array[Piece] = []
var wall_kicks

@onready var tetromino_cells = Shared.cells[tetromino_data.tetromino_type]
@onready var piece_scene = preload("res://scenes/piece.tscn")
@onready var timer: Timer = $Timer


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
		hard_drop()
	elif Input.is_action_just_pressed("rotate_left"):
		pass
	elif Input.is_action_just_pressed("rotate_right"):
		pass


func hard_drop():
	while move(Vector2.DOWN):
		continue
	lock()


func lock():
	timer.stop()
	lock_tetromino.emit(self)
	set_process_input(false)


func move(direction: Vector2) -> bool:
	var new_position = calculate_global_position(direction, global_position)
	if new_position:
		global_position = new_position
		return true
	return false


func calculate_global_position(direction: Vector2, starting_global_position: Vector2) -> Variant:
	if not is_within_game_bounds(direction, starting_global_position):
		return null
	return starting_global_position + direction * pieces[0].get_size()


func is_within_game_bounds(direction: Vector2, starting_global_position: Vector2):
	for piece in pieces:
		var new_position = piece.position + starting_global_position + direction * piece.get_size()
		if (
			new_position.x < bounds.x[0]
			|| new_position.x > bounds.x[1]
			|| new_position.y >= bounds.y[1]
		):
			return false
	return true


func _on_timer_timeout() -> void:
	var should_lock = not move(Vector2.DOWN)
	if should_lock:
		lock()
