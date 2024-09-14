extension RuntimeSpec.source.source {
  init() {
    self.Source = .init(
      a: { source, args, context, info async throws in
        fatalError("Not implemented")
      }
    )
  }
}