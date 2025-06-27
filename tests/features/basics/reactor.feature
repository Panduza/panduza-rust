Feature: Reactor Object
  Panduza client must provides an entry point object called 'reactor'.
  This object will be used to manage all operations related to Panduza connection.
  - It must allow to connect to the platform
  - It must allow to find an attribute from its name
  - It must allow to declare an attribute from its topic

  Scenario: Fail connecting to a platform
    Given a reactor trying to connect to an invalid platform
    Then the reactor returned an error

  Scenario: Fail connecting to a platform with a wrong certificate
    Given a reactor trying to connect to a platform with a wrong certificate
    Then the reactor returned an error

  Scenario: Fail connecting to a platform with an expired certificate
    Given a reactor trying to connect to a platform with an expired certificate
    Then the reactor returned an error

  Scenario: Connect to a platform
    Given a reactor connected on a test platform
    Then the reactor is successfully connected to the platform

  Scenario: Find an existing attribute from its name
    Given a reactor connected on a test platform
    Given an attribute name "boolean/rw"
    When the reactor find function is called with the previously given attribute name
    Then the reactor must return a success

  Scenario: Try to find an non existing attribute from its name
    Given a reactor connected on a test platform
    Given an attribute name "boolean/doesnotexist"
    When the reactor find function is called with the previously given attribute name
    Then the reactor must return a null value
