Feature: Panduza client must be able to manage boolean attributes

  Background:
    Given a client connected to "localhost" on port 1883

  Scenario: Client must be able to manage RW boolean attribute
    Given the attribute rw "boolean/rw"
    When I set rw boolean to true
    Then the rw boolean value is true
    When I set rw boolean to false
    Then the rw boolean value is false
