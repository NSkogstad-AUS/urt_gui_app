import socket

# Create a UDP socket
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

# Define the address and port to send the message to
server_address = ('192.168.168.137', 34254)  # Replace with the actual IP address of your Mac

# Send a test message
message = b'Test Message'
sock.sendto(message, server_address)

# Close the socket
sock.close()