extends Node

#var player_client = PlayerClient.new()

onready var camera_pivot = get_node("../CameraPivot")
onready var player = get_parent()

func _ready():
	Input.set_use_accumulated_input(false)
	
func _physics_process(_delta):
	camera_pivot.rotate_camera()
	
	var direction = Vector3.ZERO
	# Handle player input	
	if Input.is_action_pressed("move_forward"):
		direction += player.global_transform.basis.z
	if Input.is_action_pressed("move_back"):
		direction += -player.global_transform.basis.z
	if Input.is_action_pressed("move_left"):
		direction += player.global_transform.basis.x
	if Input.is_action_pressed("move_right"):
		direction += -player.global_transform.basis.x
	
	player.handle_input(direction);
	
#	$Label.text = \
#	"fps: " + String(Engine.get_frames_per_second())
#	"direction: " + String(direction) + "\n" + \
#	"camera.y: " + String($CameraPivot.rotation_degrees.y) + "\n" + \
#	"player.y: " + String(player.rotation_degrees.y) + "\n"
#	"mouse.x: " + String(tmp_x_rotation) + "\n" + \
#	"mouse.y: " + String(tmp_x_rotation) + "\n"
