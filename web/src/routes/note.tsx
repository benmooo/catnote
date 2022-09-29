import ArrowBackIcon from '@mui/icons-material/ArrowBack'
import SyncIcon from '@mui/icons-material/Sync'
import AddCircleIcon from '@mui/icons-material/AddCircle'
import TagIcon from '@mui/icons-material/Tag'
import DarkModeIcon from '@mui/icons-material/DarkMode'
import AttachFileIcon from '@mui/icons-material/AttachFile'
import ShareIcon from '@mui/icons-material/Share'
import FullscreenIcon from '@mui/icons-material/Fullscreen'
import MoreIcon from '@mui/icons-material/MoreVert'
import EditIcon from '@mui/icons-material/Edit'
import { useParams } from 'react-router-dom'

import EditorJS, { LogLevels } from '@editorjs/editorjs'
import Header from '@editorjs/header'
import Link from '@editorjs/link'
import List from '@editorjs/list'
import Quote from '@editorjs/quote'
import CheckList from '@editorjs/checklist'
import Delimiter from '@editorjs/delimiter'
import Image from '@editorjs/image'
import Code from '@editorjs/code'
import { useEffect, useRef, useState } from 'react'
import { DEFAULT_EDITOR_DATA } from '../mocks/data'

import '../editor.css'

export default () => {
  // const params = useParams()
  const EDITOR_HOLDER_ID = 'editorjs'
  const ejInstance = useRef<EditorJS>()
  const [editorData, setEditorData] = useState(DEFAULT_EDITOR_DATA)

  useEffect(() => {
    if (!ejInstance.current) {
      initEditor()
    }

    // cleanup
    return () => {
      ejInstance.current!.destroy()
    }
  }, [])

  const initEditor = () => {
    const editor = new EditorJS({
      holder: EDITOR_HOLDER_ID,
      logLevel: 'VERBOSE' as LogLevels,
      tools: {
        header: {
          class: Header,
          config: {
            levels: [1, 2, 3, 4],
            defaultLevel: 3,
          },
        },
        link: {
          class: Link,
          inlineToolbar: true,
        },
        list: {
          class: List,
        },
        quote: {
          class: Quote,
        },
        checklist: CheckList,
        delimiter: Delimiter,
        code: Code,
        image: Image,
      },
      onReady: () => {
        ejInstance.current = editor
      },
      onChange: async () => {
        let content = await editor.saver.save()
        setEditorData(content)
      },
      data: editorData,
      readOnly: false,
    })
  }

  return (
    <div className='min-h-screen bg-ink-t4 flex justify-center'>
      <div className='w-full md:w-10/12 lg:w-8/12 xl:w-6/12 bg-white shadow-md'>
        <div className='sticky top-0 flex justify-between items-center p-4 shadow-md bg-white z-10'>
          <span>
            <ArrowBackIcon sx={{ fontSize: 24 }}></ArrowBackIcon>
          </span>

          <div className='flex gap-2'>
            <span className='hover:scale-125'>
              <SyncIcon sx={{ fontSize: 24 }}></SyncIcon>
            </span>
            <span className='hover:scale-125'>
              <EditIcon sx={{ fontSize: 24 }}></EditIcon>
            </span>

            <span className='hover:scale-125'>
              <AddCircleIcon sx={{ fontSize: 24 }}></AddCircleIcon>
            </span>
            <span className='h over:scale-125'>
              <TagIcon sx={{ fontSize: 24 }}></TagIcon>
            </span>
            <span className='hover:scale-125'>
              <DarkModeIcon sx={{ fontSize: 24 }}></DarkModeIcon>
            </span>
            <span className='hover:scale-125'>
              <AttachFileIcon sx={{ fontSize: 24 }}></AttachFileIcon>
            </span>
            <span className='hover:scale-125'>
              <ShareIcon sx={{ fontSize: 24 }}></ShareIcon>
            </span>
            <span className='hover:scale-125'>
              <FullscreenIcon sx={{ fontSize: 24 }}></FullscreenIcon>
            </span>
            <span className='hover:scale-125'>
              <MoreIcon sx={{ fontSize: 24 }}></MoreIcon>
            </span>
          </div>
        </div>

        {/* note cover */}

        <div
          id='editorjs'
          className='
          p-2 
          prose-headings:font-montserrat
          prose-headings:font-medium
          prose-h1:text-3xl prose-h1:text-center
          prose-h2:text-2xl
          prose-h3:text-xl
          prose-h4:text-lg
          prose-code:bg-pink-300
          prose-code:italic
          '
        ></div>
      </div>
    </div>
  )
}
