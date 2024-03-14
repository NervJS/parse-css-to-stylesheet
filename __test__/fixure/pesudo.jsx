import { View, Text } from '@tarojs/components'
import './pesudo.scss'

export default function Pesudo() {

  return <View className='pesudo' style={{
    height: '10px',
    width: Math.random() > 0.5 ? '10px' : '20px'
  }}>
    <Text className='text'>asdasdasdasdasd</Text>
  </View>
}
