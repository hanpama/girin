// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "SchemaGeneration",
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "SchemaGeneration",
            targets: ["SchemaGeneration"]),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "SchemaGeneration"),
        .testTarget(
            name: "SchemaGenerationTests",
            dependencies: ["SchemaGeneration"]),
    ]
)
