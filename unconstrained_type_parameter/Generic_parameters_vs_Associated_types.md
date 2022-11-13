Generic parameters are like trait's "input types" - when a method is being called, it's the trait's user who gets to state them.

Associated types are like trait's "output types" - when a method is being called, it's the trait's implementer who gets to state them.

I think it's nicely evident on the Add / Sub / etc. family of traits:
```
impl Add<Point> for Point { // defines a `Point + Point` operation
type Output = Self;     // ... that returns a `Point`

    fn add(self, other: Vector) -> Self {
        /* ... */
    }
}

impl Add<Vector> for Point { // defines a `Point + Vector` operation
type Output = Self;      // ... that returns a `Point`

    fn add(self, other: Vector) -> Self {
        /* ... */
    }
}
```
User can specify whether they want to call the first impl or the second one, but they have no control over the Output - they must accept whatever's in the trait:
```
let point = Point::new(10, 10);
let vector = Vector::new(3, 3);

let result = point + vector;
//  ^ I have control over `point` and `vector` (so I can
//    choose the implementation), but at this point I can't
//    affect the fact that my implementation returns a `Point`
```