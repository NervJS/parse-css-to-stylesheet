import React from 'react'
import './Mod.scss'

function Cc() {
  return (
    <div className={boxClass.join(' ')} style={bannerStyle}>
      <span className='line1 txt'>成员123: 4000+</span>
      <div className='cnt_row4'>
        <img className='icon2' src='//img11.360buyimg.com/img/jfs/t1/175578/35/40256/1981/64f58062Fddaf1a21/f1111d9988a65ccc.png'></img>
        <span className='instruction3'>slslsl-jsj</span>
        <span className='txt3'>复制</span>
      </div>
    </div>
  )
}

export default class Mod extends React.Component {
  getDom () {
    return (
      <div className='cc'>
        <span className='line1 txt'>成员123: 4000+</span>
        <div className='cnt_row4'>
          <img className='icon2' src='//img11.360buyimg.com/img/jfs/t1/175578/35/40256/1981/64f58062Fddaf1a21/f1111d9988a65ccc.png'></img>
          <span className='instruction3'>slslsl-jsj</span>
          <span className='txt3'>复制</span>
        </div>
      </div>
    )
  }
  render () {
    return (
      <div className='mod' style={{ width: '500px', height: 800 }}>
        <div className={classnames('cnt_row')} style>
          <>
            <img
              className='icon'
              style={{
                color: 'red'
              }}
              src='//img20.360buyimg.com/img/jfs/t1/166410/12/38783/3147/64f58062Fd7737e2b/5aaf0205cd1ce175.png'
            ></img>
            <>
              <span className='line1 instruction'>超能芭比 5分钟前查看团购</span>
            </>
          </>
        </div>
        <div className='cnt_row1'>
          <img
            className='img'
            src='//img12.360buyimg.com/img/jfs/t1/100881/15/44805/18567/64f58062F1b45e0cb/caf065a7410087ce.png'
          ></img>
          <div className='cnt_col'>
            <span className='line1 instruction1'>巴拉巴拉小魔仙</span>
            <span className='line1 txt'>成员: 4000+</span>
          </div>
          <div className='cnt_row2'>
            { this.getDom() }
            <img
              className='icon1'
              src='//img11.360buyimg.com/img/jfs/t1/175578/35/40256/1981/64f58062Fddaf1a21/f1111d9988a65ccc.png'
            ></img>
            <span className='instruction2'>slslsl-jsj</span>
            <span className='txt1'>复制</span>
          </div>
        </div>
        <div className='cnt_row3'>
          <span className='line2 txt2'>
            团长介绍：售前售后进群售前售后进群售前售后进群售前售后进群VXklsidohh...
          </span>
          <img
            className='img1'
            src='//img14.360buyimg.com/img/jfs/t1/206378/24/25778/195/64eca527F378f17a2/c1623681708609fd.png'
          ></img>
        </div>
      </div>
    )
  }
}

// function Mod() {
//   return (
//     <div className='mod' style={{ width: '500px', height: 800 }}>
//       <div className='cnt_row'>
//         <img
//           className='icon'
//           src='//img20.360buyimg.com/img/jfs/t1/166410/12/38783/3147/64f58062Fd7737e2b/5aaf0205cd1ce175.png'
//         ></img>
//         <span className='line1 instruction'>超能芭比 5分钟前查看团购</span>
//       </div>
//       <div className='cnt_row1'>
//         <img
//           className='img'
//           src='//img12.360buyimg.com/img/jfs/t1/100881/15/44805/18567/64f58062F1b45e0cb/caf065a7410087ce.png'
//         ></img>
//         <div className='cnt_col'>
//           <span className='line1 instruction1'>巴拉巴拉小魔仙</span>
//           <span className='line1 txt'>成员: 4000+</span>
//         </div>
//         <div className='cnt_row2'>
//           <img
//             className='icon1'
//             src='//img11.360buyimg.com/img/jfs/t1/175578/35/40256/1981/64f58062Fddaf1a21/f1111d9988a65ccc.png'
//           ></img>
//           <span className='instruction2'>slslsl-jsj</span>
//           <span className='txt1'>复制</span>
//         </div>
//       </div>
//       <div className='cnt_row3'>
//         <span className='line2 txt2'>
//           团长介绍：售前售后进群售前售后进群售前售后进群售前售后进群VXklsidohh...
//         </span>
//         <img
//           className='img1'
//           src='//img14.360buyimg.com/img/jfs/t1/206378/24/25778/195/64eca527F378f17a2/c1623681708609fd.png'
//         ></img>
//       </div>
//     </div>
//   )
// }
