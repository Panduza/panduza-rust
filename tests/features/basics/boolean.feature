Feature: Panduza client must be able to manage boolean attributes

  Background:
    Given a client connected to "localhost" on port 1883

  Scenario: Client must return an error if the attribute topic does not exist
    # This Scenario is more for reactor find tests
    When a non existant attribute "boolean/notexist" is provided
    Then an error is return by the reactor during creation

  Scenario: Client must be able to manage RW boolean attribute
    Given the attribute rw "boolean/rw"
    When I set rw boolean to true
    Then the rw boolean value is true
    When I set rw boolean to false
    Then the rw boolean value is false

  Scenario: Client must be able to manage an error during a boolean attribute operation
    Given the attribute wo "boolean/error"
    Given the status attribute for the instance managing the wo attribute
    Then the instance status attribute must be "Running"
    When I set wo boolean to true
    Then the instance status attribute must be "Error"
