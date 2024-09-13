extension RuntimeSpec.Resolve.Schema {
  init() {
    self.GrandParent = .init(
      echo: { src, info, args async throws in
        fatalError("Not implemented")
      },
      echoSync: { src, info, args throws in
        fatalError("Not implemented")
      }
    )
    self.Parent = .init(
      echo: { src, info, args async throws in
        fatalError("Not implemented")
      },
      echoSync: { src, info, args async throws in
        fatalError("Not implemented")
      }
    )
    self.Child = .init(
      echo: { src, info, args async throws in
        fatalError("Not implemented")
      },
      echoSync: { src, info, args async throws in
        fatalError("Not implemented")
      },
      basicInterface: { src, info, args async throws in
        fatalError("Not implemented")
      },
      basicInterfaceList: { src, info, args async throws in
        fatalError("Not implemented")
      },
      basicInterfaceNonNullList: { src, info, args async throws in
        fatalError("Not implemented")
      },
      basicInterfaceNonNullListNonNullElement: { src, info, args async throws in
        fatalError("Not implemented")
      },
      basicUnion: { src, info, args async throws in
        fatalError("Not implemented")
      },
      basicUnionList: { src, info, args async throws in
        fatalError("Not implemented")
      },
      basicUnionNonNullList: { src, info, args async throws in
        fatalError("Not implemented")
      },
      basicUnionNonNullListNonNullElement: { src, info, args async throws in
        fatalError("Not implemented")
      },
      basicEnum: { src, info, args async throws in
        fatalError("Not implemented")
      },
      basicEnumList: { src, info, args async throws in
        fatalError("Not implemented")
      },
      basicEnumNonNullList: { src, info, args async throws in
        fatalError("Not implemented")
      },
      basicEnumNonNullListNonNullElement: { src, info, args async throws in
        fatalError("Not implemented")
      }
    )
  }
}