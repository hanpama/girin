extension RuntimeSpec.Resolve.Schema {
    init() {
        self.GrandParent = .init(
            echo: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            echoSync: { source, args, context, info throws in
                fatalError("Not implemented")
            }
        )
        self.Parent = .init(
            echo: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            echoSync: { source, args, context, info async throws in
                fatalError("Not implemented")
            }
        )
        self.Child = .init(
            echo: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            echoSync: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            basicInterface: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            basicInterfaceList: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            basicInterfaceNonNullList: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            basicInterfaceNonNullListNonNullElement: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            basicUnion: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            basicUnionList: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            basicUnionNonNullList: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            basicUnionNonNullListNonNullElement: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            basicEnum: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            basicEnumList: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            basicEnumNonNullList: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            basicEnumNonNullListNonNullElement: { source, args, context, info async throws in
                fatalError("Not implemented")
            }
        )
    }
}