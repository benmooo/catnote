import catnote from '../assets/images/catnote.svg'
import CopyrightIcon from '@mui/icons-material/Copyright'

export default () => {
  return (
    <div className='h-screen bg-ink-t4 flex flex-col text-center'>
      <div className='my-auto'>
        <p className='text-8xl  lg:text-[200px] flex justify-center'>
          <span className='px-2'>4</span>
          <img className='w-20 lg:w-40' src={catnote} alt='' />
          <span className='px-2'>4</span>
        </p>
        <p className='text-2xl lg:text-5xl font-light'>
          PAGE NOT FOUND
        </p>
      </div>

      <div className='font-montserrat text-xs text-center absolute inset-x-0 bottom-0 pb-2'>
        <CopyrightIcon sx={{ fontSize: 12 }}></CopyrightIcon> CATNOTE 2020
      </div>
    </div>
  )
}
