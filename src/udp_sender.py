import socket

# Create a UDP socket
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

# Define the address and port to send the message to
server_address = ('127.0.0.1', 34254)

# Send a test message
message = b'Test Message'
sock.sendto(message, server_address)

# Close the socket
sock.close()