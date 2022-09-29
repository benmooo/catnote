import MenuIcon from '@mui/icons-material/Menu'
import CloseIcon from '@mui/icons-material/Close'
import CopyrightIcon from '@mui/icons-material/Copyright'
import GithubIcon from '@mui/icons-material/GitHub'
import TwitterIcon from '@mui/icons-material/Twitter'
import InstagramIcon from '@mui/icons-material/Instagram'
import { useState } from 'react'
import logo from '../assets/images/logo.svg'
import inkSplatter from '../assets/images/ink_splatter.svg'
import slideup from '../assets/images/slideup.svg'

export default () => {
  let [open, setOpen] = useState(false)

  return (
    <div className='bg-ink-t4 min-h-screen'>
      <nav className='relative md:flex'>
        <div className='p-6 flex justify-between items-center z-30'>
          <a href='#' className='flex items-center'>
            <img src={logo} alt='catnote' className='inline w-12' />
            <span className='pl-2 font-montserrat text-2xl'>CATNOTE</span>
          </a>

          <a
            href='#'
            className='p-3 rounded-xl hover:shadow-md hover:bg-white md:hidden'
            onClick={() => {
              setOpen(!open)
            }}
          >
            {open ? (
              <CloseIcon sx={{ fontSize: 28 }}></CloseIcon>
            ) : (
              <MenuIcon sx={{ fontSize: 28 }}></MenuIcon>
            )}
          </a>
        </div>

        <ul
          className={`absolute bg-ink-t4 z-10 md:bg-auto w-full md:w-auto md:static transition-transform duration-500 ease-in-out ${
            open ? 'top-24' : 'top-[-400px]'
          } shadow-md p-6 text-right text-ink md:shadow-none md:flex md:grow md:items-center md:justify-between`}
        >
          <div className='md:flex md:ml-32'>
            <li className='mx-4'>
              <a
                href='#'
                className='font-montserrat hover:text-ink hover:text-xl duration-500'
              >
                HOME
              </a>
            </li>
            <li className='mx-4'>
              <a
                href='#'
                className='font-montserrat hover:text-ink hover:text-xl duration-500'
              >
                ABOUT
              </a>
            </li>
            <li className='mx-4'>
              <a
                href='#'
                className='font-montserrat hover:text-ink hover:text-xl duration-500'
              >
                LICENSE
              </a>
            </li>
            <li className='mx-4'>
              <a
                href='#'
                className='font-montserrat hover:text-ink hover:text-xl duration-500'
              >
                CONTACT
              </a>
            </li>
          </div>

          <div className='mx-4 pt-4 md:flex md:pt-0 md:ml-24'>
            <button className='text font-montserrat px-4 py-2 border border-ink bg-ink text-ink-t4 active:bg-white active:text-ink'>
              Login
            </button>

            <button className='ml-4 text font-montserrat px-4 py-2 border border-ink text-ink active:bg-ink active:text-ink-t4'>
              Signup
            </button>
          </div>
        </ul>
      </nav>

      {/* <div className='md:flex md:flex-row-reverse md:items-center md:justify-around md:p-6'> */}
      <div className='md:grid md:grid-cols-2 md:justify-center md:items-center'>
        <div className='md:max-w-2xl md:max-h-2xl md:pl-6'>
          <img src={inkSplatter} alt='' />
        </div>

        <div className='md:grid md:grid-cols-5 md:justify-center'>
          <div className='px-6 md:col-span-4'>
            <h2 className='md:text-4xl font-montserrat text-xl'>
              Cat Note, yet another note taking app.
            </h2>
            <p className='md:text-xl font-montserrat text-light text-ink-t1 mt-2'>
              {' '}
              Node based knowledge management
            </p>
            <button className='border border-ink px-4 py-2 text-ink font-montserrat mt-4 hover:bg-ink hover:text-ink-t4'>
              Get Start
            </button>
          </div>

          <div className='hidden md:block'>
            <img src={slideup} alt='' className='' />
            <img src={slideup} alt='' className='rotate-180 mt-16' />
          </div>
        </div>
      </div>

      <div className='hidden md:grid md:grid-cols-5 md:items-center'>
        <div className='font-montserrat text-xs text-center'>
          <CopyrightIcon sx={{ fontSize: 12 }}></CopyrightIcon> CATNOTE 2020
        </div>

        <div className='font-montserrat text-xs text-center'>By Akatsuki</div>

        <div className='bg-ink text-ink-t4 py-6 font-montserrat text-xs flex items-center justify-center'>
          <GithubIcon sx={{ fontSize: 14 }}></GithubIcon>
          <span className='pl-1'>Github</span>
        </div>

        <div className='bg-ink text-ink-t4 py-6 font-montserrat text-xs flex items-center justify-center'>
          <TwitterIcon sx={{ fontSize: 14 }}></TwitterIcon>
          <span className='pl-1'>Twitter</span>
        </div>

        <div className='bg-ink text-ink-t4 py-6 font-montserrat text-xs flex items-center justify-center'>
          <InstagramIcon sx={{ fontSize: 14 }}></InstagramIcon>
          <span className='pl-1'>Instagram</span>
        </div>
      </div>

      <div className='font-montserrat text-xs text-center absolute inset-x-0 bottom-0 pb-2 md:hidden'>
        <CopyrightIcon sx={{ fontSize: 12 }}></CopyrightIcon> CATNOTE 2020
      </div>
    </div>
  )
}
