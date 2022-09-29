import NotesIcon from '@mui/icons-material/Notes'
import TagIcon from '@mui/icons-material/TagRounded'
import FavIcon from '@mui/icons-material/Favorite'
import DeleteIcon from '@mui/icons-material/Delete'
import logo from '../assets/images/logo.svg'
export default (props: { open: boolean }) => {
  return (
    <aside
      className={`${
        props.open ? 'translate-x-0 lg:sticky' : '-translate-x-full'
      } z-20 fixed top-0 w-80 h-screen bg-ink-t4 p-6 transition-all ease-in duration-200 shadow-2xl`}
    >
      {/* <div className='h-screen  p-6'></div> */}

      <div className='flex justify-center items-center'>
        <img src={logo} alt='catnote' className='inline w-12' />
        <span className='pl-2 font-montserrat text-2xl'>CATNOTE</span>
      </div>

      <nav className='font-montserrat text-lg mt-12'>
        <ul className='flex flex-col justify-center items-center gap-y-6'>
          <li className='w-full'>
            <a
              href=''
              className='flex justify-between items-center px-12 py-2 bg-ink text-ink-t4 rounded-lg'
            >
              <span className='pr-12'>Notes</span>
              <span>
                <NotesIcon sx={{ fontSize: 24 }}></NotesIcon>
              </span>
            </a>
          </li>

          <li className='w-full'>
            <a href='' className='flex justify-between items-center px-12'>
              <span>Tags</span>
              <span>
                <TagIcon sx={{ fontSize: 24 }}></TagIcon>
              </span>
            </a>
          </li>

          <li className='w-full'>
            <a className='flex justify-between items-center px-12'>
              <span>Favs</span>
              <span>
                <FavIcon sx={{ fontSize: 24 }}></FavIcon>
              </span>
            </a>
          </li>

          <li className='w-full'>
            <a className='flex justify-between items-center px-12'>
              <span>Trash</span>
              <span>
                <DeleteIcon sx={{ fontSize: 24 }}></DeleteIcon>
              </span>
            </a>
          </li>
        </ul>
      </nav>
    </aside>
  )
}
