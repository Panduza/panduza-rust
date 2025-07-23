Feature: Logger Security
  Panduza logger must be able to only sub to attributes 

  Background:
    Given a logger reactor connected on a test platform

  Scenario: write messages
    Given the boolean attribute rw "boolean/rw"
    When I try to set rw boolean to true
    Then the rw boolean value is false

  Scenario: read messages
    Given the boolean attribute rw "boolean/rw"
    When I toglle rw boolean
    Then I receive ten messages
