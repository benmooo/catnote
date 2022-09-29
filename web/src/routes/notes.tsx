import SearchIcon from '@mui/icons-material/Search'
import MenuOpenIcon from '@mui/icons-material/ArrowBack'
import AddIcon from '@mui/icons-material/AddCircleRounded'
import SearchModal from '../components/SearchModal'
import Sidebar from '../components/Sidebar'
import { useState } from 'react'
import { Outlet } from 'react-router-dom'
import NoteCard from '../components/NoteCard'

export default () => {
  let [open, setOpen] = useState(false)
  return (
    <div className='flex'>
      <Sidebar open={open}></Sidebar>

      <main className='p-6 grow bg-ink-t4'>
        <div className='flex justify-between items-center'>
          <div className='cursor-pointer'>
            <a
              onClick={() => {
                setOpen(!open)
              }}
              className='cursor-pointer mr-12 text-ink-t1'
            >
              <MenuOpenIcon sx={{ fontSize: 32 }}></MenuOpenIcon>
            </a>

            <span>
              <SearchIcon sx={{ fontSize: 32 }}></SearchIcon>
            </span>
            <span className='font-montserrat text-ink-t3 pl-2 text-lg'>
              Search..{' '}
              <span className='hidden md:inline ml-2'>
                Ctrl+<span className='text-ink-t1'>K</span>
              </span>
            </span>
          </div>
          <div className='font-montserrat text-xl flex items-center'>
            <span className='mr-2'>Create Note</span>
            <span className='cursor-pointer'>
              <AddIcon sx={{ fontSize: 32 }}></AddIcon>
            </span>
          </div>
        </div>

        <div className='pt-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2'>
          {[...Array(22).keys()].map((i) => (
            <NoteCard key={i}></NoteCard>
          ))}
        </div>
        {/* <Outlet></Outlet> */}
      </main>

      <SearchModal show={false}></SearchModal>
    </div>
  )
}
