import TimerIcon from '@mui/icons-material/Timer'
export default () => {
  return (
    <div className='hover:shadow-md hover:bg-ink-t3'>
      <div className='p-2 border-b border-b-ink-t1'>
        <p className='font-montserrat font-medium text-lg'>LOREM isump</p>
        <p className='font-light text-ink-t1 line-clamp-3 text-sm'>
          Lorem ipsum, dolor sit amet consectetur adipisicing elit. Quas mollitia recusandae ex quasi illum! Molestiae, fugit! Alias, voluptates. Atque, beatae minus. Quaerat quisquam corrupti laborum vel provident cum voluptatem nam!
        </p>

        <p className='flex pt-4 gap-2'>
          <span className='px-2 border border-ink font-montserrat text-sm  rounded-xl'>
            Math
          </span>
          <span className='px-2 border border-ink font-montserrat text-sm  rounded-xl'>
            Bio
          </span>
          <span className='px-2 border border-ink font-montserrat text-sm  rounded-xl'>
            CG
          </span>
        </p>

        <p className='flex justify-end text-ink-t1'>
          <span className=''>
            <TimerIcon sx={{ fontSize: 16 }}></TimerIcon>
          </span>
          <span className='font-light'>2d</span>
        </p>
      </div>
    </div>
  )
}
