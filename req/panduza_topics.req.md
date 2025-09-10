# Panduza Topics Format

The topic format is structured as follows:

`{namespace}/pza/{instance}/{*class}/{attribute}/[cmd|att]`

### Components

1. **`{namespace}`**:
   - **Description**: This segment represents a namespace, which is used to group related items and avoid naming conflicts. It typically denotes the name of an organization, project, or specific domain. It is
   an optional element.
   - **Example**: `company`, `project_alpha`
2. **`pza`**:
   - **Description**: This is a fixed identifier used to designate a specific type of resource or service within the system.
   - **Note**: The identifier `pza` is constant and should always be included as shown.
3. **`{instance}`**:
   - **Description**: This segment refers to a specific instance within the namespace. An instance is a particular occurrence of a resource or service.
   - **Example**: `instance_1`, `service_001`
4. **`{*class}`**:
   - **Description**: This segment represents multiple levels of classes. It indicates that this part of the topic can contain a hierarchy of classes or categories, allowing for a deeper and more detailed structuring of resources.
   - **Example**: `level1/level2/level3`
5. **`{attribute}`**:
   - **Description**: This segment specifies a particular attribute of the class or instance. An attribute is a property or characteristic of an object.
   - **Example**: `color`, `size`
6. **`[cmd|att]`**:
   - **Description**: This segment indicates that the topic must end with either a command (`cmd`) or an attribute (`att`). This means the "topic" must provide either a command or an attribute to complete the topic.
   - **Example**: `cmd`, `att`

### Example topic

Here is an example of a complete topic using the described format:
`company/pza/service_001/level1/level2/color/cmd`
In this example:
- `company` is the namespace.
- `pza` is the fixed identifier.
- `service_001` is the instance.
- `level1/level2` are the hierarchical class levels.
- `color` is the attribute.
- `cmd` is the command that completes the path.

Here is an example without namespace:
`pza/service_001/level1/level2/color/cmd`
