import { reinterpret } from 'ref-napi'

export const byteBufferToBuffer = (buffer: { data: Buffer; len: number }) => reinterpret(buffer.data, buffer.len)

export const secretBufferToBuffer = byteBufferToBuffer
