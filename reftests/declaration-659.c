#pragma gnu
int typedef * foo, baz[static 10][const *];

/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier Int
    DeclarationSpecifier
        StorageClassSpecifier Typedef
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "foo"
            DerivedDeclarator
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "baz"
            DerivedDeclarator
                ArrayDeclarator
                    ArraySize StaticExpression
                        Expression
                            Constant
                                Integer "10"
                                    IntegerBase Decimal
                                    IntegerSuffix false false
                                        IntegerSize Int
            DerivedDeclarator
                ArrayDeclarator
                    TypeQualifier Const
                    ArraySize VariableUnknown
===*/
