use byteorder::LittleEndian;

pub type Vector3d<T = f32> = (T, T, T);

pub type Sphere3d<T = f32> = (Vector3d<T>, T);

pub type Matrix3d<T = f32> = (Vector3d<T>, Vector3d<T>, Vector3d<T>, Vector3d<T>);

pub type U32Bytes = (u8, u8, u8, u8);

pub type SpawnByteOrder = LittleEndian;
