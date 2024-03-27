import { View, Text } from '@tarojs/components'
import './pesudo.scss'

export default function Pesudo() {

  return <>
    <View style={{width: '100px'}}></View>
    <View style={{width: `${'100px'}`}}></View>
    <View style={{width: `${'100px'}`, height: '100px'}}></View>
    <View className='a'></View>
    <View className={'a' + 1}></View>
    <View style={{width: '100px'}} className='a'></View>
    <View style={{width: `${'100px'}`}} className='a'></View>
    <View style={{width: `${'100px'}`, height: '100px'}} className='a'></View>
  </>
}
