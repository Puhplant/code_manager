## Guidance for implementing feature in the api

1. Implement one end point at a time, do not be tempted to implement many at once, instead stop for feedback
2. The pattern to follow is creating a controller that returns the body a Validatable<> struct that should have all fields as Options<>
3. The Validatable<> struct should be validated and return a struct to be used in the controller. Find examples of this validation done before and copy the pattern
4. The controller endpoint should create a scope which creates and calls the main service for the endpoint
5. The main service should call other services to complete its work. Any db calls should be encapsulated into their own separate service
6. Services should be registered in the Scope Container for DI
7. Db calls should use the Clorinde methods that have already been created. Do not modify the clorinde directory in any way
8. Look for services that can be reused before creating new ones