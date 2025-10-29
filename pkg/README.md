# Lisa 3D Polygons: Spherical Coordinate Lissajous Curves

## Overview

This project implements **Spherical Coordinate Lissajous Curves** - a 3D extension of classical Lissajous curves that live on the surface of a sphere. These curves create beautiful, complex 3D patterns by varying polar and azimuthal angles according to Lissajous principles.

## Mathematical Foundation

### Traditional 2D Lissajous Curves

A classic 2D Lissajous curve is defined as:
```
x(t) = A * sin(a*t + δ)
y(t) = B * sin(b*t)
```

### Spherical Coordinate Lissajous Curves

Our implementation extends this concept to 3D using spherical coordinates:

```rust
position(t) = R * [
    sin(A*t) * cos(B*t),  // x = r * sin(θ) * cos(φ)
    sin(A*t) * sin(B*t),  // y = r * sin(θ) * sin(φ)  
    cos(A*t)              // z = r * cos(θ)
]
```

Where:
- `R` is the sphere radius  
- `θ = A*t` is the polar angle (colatitude, measured from north pole)
- `φ = B*t` is the azimuthal angle (longitude, rotation around z-axis)
- This creates **true spherical coordinate Lissajous curves** where all points lie exactly on a sphere of radius R

## Key Properties

### 1. **Spherical Constraint**
- All points lie exactly on a sphere of radius `R`
- The curve never leaves the spherical surface
- Creates 3D Lissajous patterns "wrapped" around a sphere

### 2. **Angular Frequencies**
- **A**: Controls polar angle oscillation rate (north-south movement)
- **B**: Controls azimuthal angle oscillation rate (east-west rotation)
- **A:B ratio**: Determines the complexity and periodicity of the pattern

### 3. **Geometric Behavior**
- **A=1, B=1**: Simple circular path around the sphere
- **A=2, B=1**: Figure-8 pattern on the sphere
- **A=3, B=2**: Complex rosette pattern with 3:2 frequency ratio

## Trihedron Analysis

The implementation computes a moving coordinate system (trihedron) along the curve:

### 1. **D1: Tangent Vector** (First derivative)
```
D1(t) = normalize([
    A*cos(A*t)*cos(B*t) - B*sin(A*t)*sin(B*t),  // d/dt(sin(θ)cos(φ))
    A*cos(A*t)*sin(B*t) + B*sin(A*t)*cos(B*t),  // d/dt(sin(θ)sin(φ))
    -A*sin(A*t)                                  // d/dt(cos(θ))
])
```

### 2. **D2: Normal Vector** (Position × D1)
```
D2(t) = normalize(position(t) × D1(t))
```

### 3. **D3: Binormal Vector** (D1 × D2)
```
D3(t) = D1(t) × D2(t)
```

This creates a **Frenet-like frame** that follows the curve, allowing polygons to be positioned and oriented naturally along the path.

## Polygon Transformation

Each polygon is transformed using a 4×4 matrix that:

1. **Rotates** the polygon to align with the local trihedron
2. **Translates** it to the curve position at parameter `t`

```
Transform Matrix = Translation * Rotation
```

Where the rotation matrix uses the trihedron vectors as columns:
```
Rotation = [D2 | D3 | D1 | 0]
           [0  | 0  | 0  | 1]
```

### Polygon Orientation Verification
- **Initial polygon**: Created in XY plane (Z=0)
- **After transformation**:
  - X-axis (radial) → D2 (normal direction)
  - Y-axis (tangential) → D3 (binormal direction)  
  - Z-axis (polygon normal) → D1 (curve tangent)
- **Result**: Polygons are **perpendicular to D1** and lie in the **D2-D3 plane**

### Orthogonality Testing
The implementation includes verification functions that prove:
- **D1·D2 ≈ 0** (tangent ⊥ normal)
- **D1·D3 ≈ 0** (tangent ⊥ binormal)
- **D2·D3 ≈ 0** (normal ⊥ binormal)
- **D3 = D1 × D2** (right-handed coordinate system)

## Color Generation

Colors vary with the parameter `t` using:
```
R(t) = 0.5 + 0.5 * sin(t)
G(t) = 0.5 - 0.5 * sin(t)  
B(t) = 0.5 + 0.5 * cos(t)
Alpha = 0.35 (constant)
```

This creates a smooth color transition around the curve.

## Parameter Analysis

For the default parameters `A=3, B=2, R=100`:

### Frequency Behavior
- **A=3**: Controls the polar angle θ with 3 cycles, creating 3 oscillations north-south on the sphere
- **B=2**: Controls the azimuthal angle φ with 2 cycles, creating 2 rotations around the z-axis
- The **3:2 ratio** creates a complex pattern with 6 lobes before repeating

### Curve Characteristics
- **Spherical constraint**: All points lie exactly on a sphere of radius R=100
- **Periodic**: The curve repeats after `t = 2π * lcm(A,B) / (A*B) = 2π`
- **3D structure**: Full utilization of all three spatial dimensions on the sphere surface

## Relationship to Traditional Lissajous Curves

This implementation creates **spherical Lissajous curves**, which differ from traditional planar Lissajous curves in several ways:

1. **Spherical constraint**: All points are constrained to lie on a sphere of radius R
2. **Polar coordinate parametrization**: Uses spherical coordinates (θ, φ) rather than Cartesian (x, y)
3. **3D nature**: The curve fully utilizes three spatial dimensions
4. **Geometric interpretation**: Rather than creating figure-8 patterns in a plane, it creates complex looping patterns on a spherical surface

## Camera System

The visualization uses a **curve-following camera** that provides a first-person experience traveling along the Lissajous curve:

### Camera Configuration at Parameter t
- **Camera position (eye)**: `position(t)` - exactly on the Lissajous curve
- **Look target**: `position(t) + D1(t)` - one unit ahead in the tangent direction
- **Up vector**: `D2(t)` - the normal vector perpendicular to the curve

### Mathematical Verification
- **Look direction**: `(Look target - Eye) = D1(t)` - perfectly aligned with curve tangent
- **Camera orientation**: Forms right-handed coordinate system with trihedron
- **Speed parameter**: Controls rate of travel along curve (JavaScript configurable)

This creates an immersive view where you travel along the curve path with the viewing direction always tangent to the curve.

## Visual Characteristics

The resulting visualization shows:
- A **spherical Lissajous curve** that forms complex looping patterns on the sphere surface
- **Polygons oriented** according to the local trihedron (tangent, normal, binormal vectors)
- **Smooth color transitions** creating a rainbow effect based on parameter t
- **First-person curve traversal** with mathematically accurate camera alignment

## Mathematical Significance

This implementation demonstrates several advanced concepts:

1. **Spherical parametric curves**: Extension of classical Lissajous to spherical surfaces
2. **Differential geometry**: Computation of tangent vectors and moving coordinate frames
3. **Linear algebra**: 4×4 transformation matrices for positioning and orienting polygons
4. **Real-time visualization**: WebGL rendering of complex 3D mathematical objects

The spherical Lissajous curves create beautiful, symmetric patterns that are both mathematically rigorous and visually striking, representing a natural 3D extension of the classical 2D Lissajous family.

## Conclusion

The "Lisa 3D" system implements **spherical coordinate Lissajous curves** - a true 3D extension of classical Lissajous curves that constrains all points to a spherical surface. This creates visually striking results while maintaining mathematical rigor through:

- **Spherical parametric curve generation** using polar and azimuthal angles
- **Moving trihedron calculation** for local coordinate systems
- **4×4 transformation matrices** for polygon positioning and orientation  
- **Real-time 3D visualization** with WebGL rendering

The implementation successfully bridges classical 2D Lissajous theory with modern 3D computational geometry, creating both mathematically sound and visually compelling results. The spherical constraint produces elegant looping patterns that are a natural and beautiful extension of the traditional Lissajous family.