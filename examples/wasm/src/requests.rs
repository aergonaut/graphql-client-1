#[derive(GraphQLQuery)]
#[GraphQLQuery(schema_path = "schema.json", query_path = "src/query.graphql")]
pub struct StationQuery;
