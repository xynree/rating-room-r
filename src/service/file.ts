import { BaseDirectory, removeFile, writeBinaryFile } from '@tauri-apps/api/fs'
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { appDataDir, join } from '@tauri-apps/api/path'

export function getFileType(file: File): string {
  return '.' + file.name.split('.').slice(-1)[0]
}

export async function saveFile(file: Blob | File): Promise<string> {
  let binaryInput = await file.arrayBuffer()
  let generatedFileName

  if (file instanceof File) {
    generatedFileName = crypto.randomUUID() + getFileType(file)
  } else generatedFileName = crypto.randomUUID() + '.jpg'

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
  if (img_path != 'default.png') {
    await removeFile(`imgs/${img_path}`, { dir: BaseDirectory.AppData })
  }
}
