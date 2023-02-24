import { BaseDirectory, writeBinaryFile } from "@tauri-apps/api/fs"
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { appDataDir, join } from "@tauri-apps/api/path"

export async function saveFile(file: File):Promise<string> {
  let binaryInput = await file.arrayBuffer();
  await writeBinaryFile(
    { path: `imgs/${file.name}`, contents: binaryInput },
    { dir: BaseDirectory.AppData }
  )
  return file.name
}

export async function imgURL(img_path:string):Promise<string> {
  const appDataDirPath = await appDataDir()
  const imgPath = await join(appDataDirPath, `imgs/${img_path}`)
  return convertFileSrc(imgPath)
}

