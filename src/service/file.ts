import { BaseDirectory, writeBinaryFile } from "@tauri-apps/api/fs"

export async function saveFile(file: File):Promise<string> {
  let binaryInput = await file.arrayBuffer();
  await writeBinaryFile(
    { path: `imgs/${file.name}`, contents: binaryInput },
    { dir: BaseDirectory.AppData }
  )
  return file.name
}
