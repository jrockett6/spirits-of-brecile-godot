extends KinematicBody

export var speed = 20
export var fall_speed = 70

var tmp_x_rotation = 0
var tmp_y_rotation = 0

var prev_cursor_x = 0
var prev_cursor_y = 0

#var client_network = ClientNetwork.new()

func _ready():
	Input.set_use_accumulated_input(false)
	
func _physics_process(delta):
	rotate_camera()
	
	var direction = Vector3.ZERO
	# Handle player input	
	if Input.is_action_pressed("move_forward"):
		direction += global_transform.basis.z
	if Input.is_action_pressed("move_back"):
		direction += -global_transform.basis.z
	if Input.is_action_pressed("move_left"):
		direction += global_transform.basis.x
	if Input.is_action_pressed("move_right"):
		direction += -global_transform.basis.x
	
#	client_network.handle_input($".", direction, delta)
	
	$Label.text = \
	"fps: " + String(Engine.get_frames_per_second())
	"direction: " + String(direction) + "\n" + \
	"camera.y: " + String($CameraPivot.rotation_degrees.y) + "\n" + \
	"player.y: " + String(rotation_degrees.y) + "\n"
#	"mouse.x: " + String(tmp_x_rotation) + "\n" + \
#	"mouse.y: " + String(tmp_x_rotation) + "\n"

#func _process(delta):
#	rotate_camera()

func _unhandled_input(event):
	if (event is InputEventMouseMotion and 
	(Input.is_action_pressed("camera_turn") or 
	Input.is_action_pressed("camera_pan"))):
		tmp_x_rotation += event.position.x - prev_cursor_x
		tmp_y_rotation += event.position.y - prev_cursor_y
	if (event is InputEventMouseMotion):
		$Label2.text = \
		String(event.position.x - prev_cursor_x) + "\n" + \
		String(event.position.y - prev_cursor_y) + "\n" + \
		"x: " + String(event.relative.x) + \
		", y: " + String(event.relative.y) + "\n" + \
		String(event.position)
		
		prev_cursor_x = event.position.x
		prev_cursor_y = event.position.y

func rotate_camera():
	var y_scale = .5
	var x_scale = .5
	
	if Input.is_action_just_pressed("camera_turn"):
		rotation_degrees.y += $CameraPivot.rotation_degrees.y
		reset_tmp_rotation()
	if Input.is_action_pressed("camera_turn"):
		$CameraPivot.rotation_degrees.x += tmp_y_rotation * y_scale
		$CameraPivot.rotation_degrees.y = 0
		rotation_degrees.y -= tmp_x_rotation * x_scale
		reset_tmp_rotation()
	elif Input.is_action_pressed("camera_pan"):
		$CameraPivot.rotation_degrees.x += tmp_y_rotation * y_scale
		$CameraPivot.rotation_degrees.y -= tmp_x_rotation * x_scale
		reset_tmp_rotation()

func reset_tmp_rotation():
	tmp_x_rotation = 0
	tmp_y_rotation = 0

