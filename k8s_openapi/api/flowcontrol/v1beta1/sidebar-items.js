initSidebarItems({"enum":[["ReadFlowSchemaResponse","Use `<ReadFlowSchemaResponse as Response>::try_from_parts` to parse the HTTP response body of [`FlowSchema::read_flow_schema`]"],["ReadFlowSchemaStatusResponse","Use `<ReadFlowSchemaStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`FlowSchema::read_flow_schema_status`]"],["ReadPriorityLevelConfigurationResponse","Use `<ReadPriorityLevelConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityLevelConfiguration::read_priority_level_configuration`]"],["ReadPriorityLevelConfigurationStatusResponse","Use `<ReadPriorityLevelConfigurationStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityLevelConfiguration::read_priority_level_configuration_status`]"]],"struct":[["FlowDistinguisherMethod","FlowDistinguisherMethod specifies the method of a flow distinguisher."],["FlowSchema","FlowSchema defines the schema of a group of flows. Note that a flow is made up of a set of inbound API requests with similar attributes and is identified by a pair of strings: the name of the FlowSchema and a “flow distinguisher”."],["FlowSchemaCondition","FlowSchemaCondition describes conditions for a FlowSchema."],["FlowSchemaSpec","FlowSchemaSpec describes how the FlowSchema’s specification looks like."],["FlowSchemaStatus","FlowSchemaStatus represents the current state of a FlowSchema."],["GroupSubject","GroupSubject holds detailed information for group-kind subject."],["LimitResponse","LimitResponse defines how to handle requests that can not be executed right now."],["LimitedPriorityLevelConfiguration","LimitedPriorityLevelConfiguration specifies how to handle requests that are subject to limits. It addresses two issues:"],["NonResourcePolicyRule","NonResourcePolicyRule is a predicate that matches non-resource requests according to their verb and the target non-resource URL. A NonResourcePolicyRule matches a request if and only if both (a) at least one member of verbs matches the request and (b) at least one member of nonResourceURLs matches the request."],["PolicyRulesWithSubjects","PolicyRulesWithSubjects prescribes a test that applies to a request to an apiserver. The test considers the subject making the request, the verb being requested, and the resource to be acted upon. This PolicyRulesWithSubjects matches a request if and only if both (a) at least one member of subjects matches the request and (b) at least one member of resourceRules or nonResourceRules matches the request."],["PriorityLevelConfiguration","PriorityLevelConfiguration represents the configuration of a priority level."],["PriorityLevelConfigurationCondition","PriorityLevelConfigurationCondition defines the condition of priority level."],["PriorityLevelConfigurationReference","PriorityLevelConfigurationReference contains information that points to the “request-priority” being used."],["PriorityLevelConfigurationSpec","PriorityLevelConfigurationSpec specifies the configuration of a priority level."],["PriorityLevelConfigurationStatus","PriorityLevelConfigurationStatus represents the current state of a “request-priority”."],["QueuingConfiguration","QueuingConfiguration holds the configuration parameters for queuing"],["ReadFlowSchemaOptional","Optional parameters of [`FlowSchema::read_flow_schema`]"],["ReadFlowSchemaStatusOptional","Optional parameters of [`FlowSchema::read_flow_schema_status`]"],["ReadPriorityLevelConfigurationOptional","Optional parameters of [`PriorityLevelConfiguration::read_priority_level_configuration`]"],["ReadPriorityLevelConfigurationStatusOptional","Optional parameters of [`PriorityLevelConfiguration::read_priority_level_configuration_status`]"],["ResourcePolicyRule","ResourcePolicyRule is a predicate that matches some resource requests, testing the request’s verb and the target resource. A ResourcePolicyRule matches a resource request if and only if: (a) at least one member of verbs matches the request, (b) at least one member of apiGroups matches the request, (c) at least one member of resources matches the request, and (d) either (d1) the request does not specify a namespace (i.e., `Namespace==\"\"`) and clusterScope is true or (d2) the request specifies a namespace and least one member of namespaces matches the request’s namespace."],["ServiceAccountSubject","ServiceAccountSubject holds detailed information for service-account-kind subject."],["Subject","Subject matches the originator of a request, as identified by the request authentication system. There are three ways of matching an originator; by user, group, or service account."],["UserSubject","UserSubject holds detailed information for user-kind subject."]]});