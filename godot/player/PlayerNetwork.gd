extends Node

var connection = StreamPeerTCP.new()

func _ready():
	connection.connect_to_host("localhost", 7077)
	


