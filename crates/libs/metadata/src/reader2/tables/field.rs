use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Field(pub Row);

// impl Field {
//     pub fn flags(&self, scope: &Reader) -> FieldAttributes {
//         FieldAttributes(scope.usize(self.0, 0))
//     }
//     pub fn name<'a>(&self, scope: &'a Reader) -> &'a str {
//         scope.str(self.0, 1)
//     }
//     pub fn constant(&self, scope: &Reader) -> Option<Constant> {
//         scope.equal_range(self.0, TABLE_CONSTANT, 1, HasConstant::Field(*self).encode()).map(Constant).next()
//     }
//     pub fn attributes(&self, scope: &Reader) -> impl Iterator<Item = Attribute> {
//         scope.attributes(self.0, HasAttribute::Field(*self))
//     }
//     pub fn ty(&self, scope: &Reader, enclosing: Option<TypeDef>) -> Type {
//         let mut blob = scope.blob(self.0, 2);
//         blob.read_usize();
//         blob.read_modifiers();
//         let def = scope.type_from_blob(&mut blob, enclosing, &[]).expect("Type not found");

//         if self.is_const(scope) {
//             def.to_const()
//         } else {
//             def
//         }
//     }
//     pub fn is_const(&self, scope: &Reader) -> bool {
//         self.attributes(scope).any(|attribute| attribute.name(scope) == "ConstAttribute")
//     }
// }