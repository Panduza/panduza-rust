Feature: Reactor Object
  Panduza client must provides an entry point object called 'reactor'.
  This object will be used to manage all operations related to Panduza connection.
  - It must allow to connect to the platform
  - It must allow to find an attribute from its name
  - It must allow to declare an attribute from its topic

  Background:
    Given a client connected on a test platform

  Scenario: Find an existing attribute from its name
    Given an attribute name "boolean/rw"
    When the reactor find function is called with the previously given attribute name
    Then the reactor must return a success

  Scenario: Try to find an non existing attribute from its name
    Given an attribute name "boolean/doesnotexist"
    When the reactor find function is called with the previously given attribute name
    Then the reactor must return an error
