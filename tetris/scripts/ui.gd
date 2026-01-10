extends CanvasLayer
class_name UI

@onready var center_container: CenterContainer = $CenterContainer

func _ready() -> void:
	center_container.hide()

func show_game_over():
	center_container.show()


func _on_button_pressed() -> void:
	get_tree().reload_current_scene()


func _on_button_pressed_quit() -> void:
	get_tree().quit()
