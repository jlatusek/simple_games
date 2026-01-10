extends Node

@onready var board = $"../Board" as Board
var next_tetromino: Shared.Tetromino
var current_tetromino: Shared.Tetromino
@onready var panel_container: PanelContainer = $"../PanelContainer"
@onready var ui: CanvasLayer = $"../UI" as UI
var is_game_over = false


func _ready() -> void:
	current_tetromino = Shared.Tetromino.values().pick_random()
	next_tetromino = Shared.Tetromino.values().pick_random()
	board.spawn_tetromino(current_tetromino, false, null)
	board.spawn_tetromino(next_tetromino, true, panel_container.size / 2)
	board.tetromino_locked.connect(on_tetromino_locked)
	board.game_over.connect(on_game_over)


func on_tetromino_locked():
	if is_game_over:
		return
	current_tetromino = next_tetromino
	next_tetromino = Shared.Tetromino.values().pick_random()
	board.spawn_tetromino(current_tetromino, false, null)
	board.spawn_tetromino(next_tetromino, true, panel_container.size / 2)

func on_game_over():
	is_game_over = true
	ui.show_game_over()
