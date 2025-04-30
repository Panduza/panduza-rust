Feature: The driver TCP in Panduza platform must be able to transmit information between the client and the tcp server

  Background:
    Given the client is connected to "127.0.0.1" on port 12345
    Given the attribute rx "serial-stream/RX"
    Given the attribute tx "serial-stream/TX"
  
  Scenario: Send a hello message
    When the client sends "hello"
    Then the client should receive "echo : hello"

  Scenario: Send a reboot command
    When the client sends "reboot"
    Then the client should receive "reboot succeeded"