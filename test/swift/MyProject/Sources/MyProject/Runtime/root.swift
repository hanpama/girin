extension RuntimeSpec.root {
  init() {
    self.Query = .init(
    )
    self.Mutation = .init(
    )
    self.DateTime = .init(
      serialize: { val in
        fatalError("Not implemented")
      },
      parseValue: { val in
        fatalError("Not implemented")
      },
      parseLiteral: { val in
        fatalError("Not implemented")
      }
    )
  }
}