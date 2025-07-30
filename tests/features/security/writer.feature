Feature: Writer Security
  Panduza writer must be able to pub and sub to attributes expect on platform topic

  Background:
    Given a writer reactor connected on a test platform

  Scenario: Write messages
    Given the writer boolean attribute wo "boolean/wo"
    Given the writer boolean attribute ro "boolean/ro"
    When I set writer wo boolean to true
    Then the ro writer boolean value is true

  Scenario: read messages
    Given the boolean attribute rw "boolean/rw"
    When I toglle rw boolean
    Then I receive ten messages

  Scenario: Write on platform topic
    When I modify structure attribute
    Then the structure attribute is not modified
