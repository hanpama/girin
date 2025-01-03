import GraphQL
import NIO
import Testing

@testable import MyProject

@Test func testExample() async throws {
  let runtime = Runtime.init(
    wiring: .init(),
    encoder: .init(),
    decoder: .init()
  )

  let schema = runtime.schema

  let query = """
    query {
      version
    }
    """

  let eventGroup = MultiThreadedEventLoopGroup(numberOfThreads: 1)

  struct Root: SourceSpec.QuerySource {
    var version: String {
      return "0.1.0"
    }
  }

  let graphqlResult = try await graphql(
    schema: schema, request: query,
    rootValue: Root.init(),
    eventLoopGroup: eventGroup
  )

  #expect(graphqlResult.errors.count == 0)

  let got: Map = graphqlResult.data!
  let expected: Map = ["version": "0.1.0"]

  #expect(got == expected)
}
