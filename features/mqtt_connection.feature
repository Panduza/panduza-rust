Feature: Mqtt Connection
  As a user of the Panduza library
  I want to connect to an MQTT broker
  So that I can send and receive messages

  Scenario: Connect to an MQTT broker
    Given A broker is running
    When I connect to the broker with the reactor
    Then I should be connected to the broker
