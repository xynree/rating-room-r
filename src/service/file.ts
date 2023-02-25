import { BaseDirectory, removeFile, writeBinaryFile } from '@tauri-apps/api/fs'
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { appDataDir, join } from '@tauri-apps/api/path'

export async function saveFile(file: File): Promise<string> {
  let binaryInput = await file.arrayBuffer()
  const fileType = file.name.split('.').slice(-1)[0]
  const generatedFileName = crypto.randomUUID() + '.' + fileType
  await writeBinaryFile(
    { path: `imgs/${generatedFileName}`, contents: binaryInput },
    { dir: BaseDirectory.AppData }
  )
  return generatedFileName
}

export async function imgURL(img_path: string): Promise<string> {
  const appDataDirPath = await appDataDir()
  const imgPath = await join(appDataDirPath, `imgs/${img_path}`)
  return convertFileSrc(imgPath)
}

export async function deleteImgFromPath(img_path: string): Promise<void> {
  await removeFile(`imgs/${img_path}`, { dir: BaseDirectory.AppData })
}
