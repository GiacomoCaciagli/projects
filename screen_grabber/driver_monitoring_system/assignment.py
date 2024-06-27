#**************************************************************************************

#
#   Driver Monitoring Systems using AI (code sample)
#
#   File: eyes_position.m
#   Author: Jacopo Sini
#   Company: Politecnico di Torino
#   Date: 19 Mar 2024
#

#**************************************************************************************
# 1 - Import the needed libraries

import cv2
import mediapipe as mp
import numpy as np 
import time
import statistics as st
import os
import math

# 2 - Set the desired setting

mp_face_mesh = mp.solutions.face_mesh
face_mesh = mp_face_mesh.FaceMesh(
    max_num_faces=1,
    refine_landmarks=True, # Enables  detailed eyes points
    min_detection_confidence=0.5,
    min_tracking_confidence=0.5
)

mp_drawing_styles = mp.solutions.drawing_styles
mp_drawing = mp.solutions.drawing_utils
drawing_spec = mp_drawing.DrawingSpec(thickness=1, circle_radius=1)

# Get the list of available capture devices (comment out)
#index = 0
#arr = []
#while True:
#    dev = cv2.VideoCapture(index)
#    try:
#        arr.append(dev.getBackendName)
#    except:
#        break
#    dev.release()
#    index += 1
#print(arr)

# 3 - Open the video source

cap = cv2.VideoCapture(0) # Local webcam (index start from 0)

# 4 - Iterate (within an infinite loop)
right_eye_idx = [33, 133, 144, 153, 158, 160]
left_eye_idx = [263, 362, 373, 380, 385, 387]
tempo = 0
start_setup = time.time()
r_eye_width_avg_np = list()
r_eye_height_avg_np = list()
l_eye_width_avg_np = list()
l_eye_height_avg_np = list()
setup = True
fixed_size = list()
while cap.isOpened(): 
    # 4.1 - Get the new frame
    success, image = cap.read() 
    start = time.time()
    # Also convert the color space from BGR to RGB

    if image is None:
        break
        #continue
    #else: #needed with some cameras/video input format
        #image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
    
    # To improve performace
    image.flags.writeable = False
    
    # 4.2 - Run MediaPipe on the frame
    results = face_mesh.process(image)

    # To improve performance
    image.flags.writeable = True

    # Convert the color space from RGB to BGR
    #image = cv2.cvtColor(image, cv2.COLOR_RGB2BGR)

    img_h, img_w, img_c = image.shape
    point_RER = [] # Right Eye Right
    point_REB = [] # Right Eye Bottom
    point_REL = [] # Right Eye Left
    point_RET = [] # Right Eye Top
    point_LER = [] # Left Eye Right
    point_LEB = [] # Left Eye Bottom
    point_LEL = [] # Left Eye Left
    point_LET = [] # Left Eye Top
    point_REIC = [] # Right Eye Iris Center
    point_LEIC = [] # Left Eye Iris Center
    point_STR = []
    point_SBR = []
    point_STL = []
    point_SBL = []
    point_52 = []

    face_2d = []
    face_3d = []
    left_eye_2d = []
    left_eye_3d = []
    right_eye_2d = []
    right_eye_3d = []
    # 4.3 - Get the landmark coordinates
    if results.multi_face_landmarks:
        for face_landmarks in results.multi_face_landmarks:
            for idx, lm in enumerate(face_landmarks.landmark):
                # Eye Gaze (Iris Tracking)
                # Left eye indices list
                #LEFT_EYE =[ 362, 382, 381, 380, 374, 373, 390, 249, 263, 466, 388, 387, 386, 385,384, 398 ]
                # Right eye indices list
                #RIGHT_EYE=[ 33, 7, 163, 144, 145, 153, 154, 155, 133, 173, 157, 158, 159, 160, 161 , 246 ]
                #LEFT_IRIS = [473, 474, 475, 476, 477]
                #RIGHT_IRIS = [468, 469, 470, 471, 472]
                if idx == 52:
                   point_52= (lm.x * img_w, lm.y * img_h) 
                   #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 0, 255), thickness=-1)
                if idx == 23:
                   #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 0, 255), thickness=-1)
                   point_SBR = (lm.x * img_w, lm.y * img_h)  
                if idx == 27:
                   #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 0, 255), thickness=-1)
                   point_STR = (lm.x * img_w, lm.y * img_h)   
                if idx == 253:
                   #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 0, 255), thickness=-1)
                   point_SBL = (lm.x * img_w, lm.y * img_h) 
                if idx == 257:
                   #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 0, 255), thickness=-1)
                   point_STL = (lm.x * img_w, lm.y * img_h) 
                if idx == 33:
                    point_RER = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 0, 255), thickness=-1)
                if idx == 145:
                    point_REB = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 0, 255), thickness=-1)
                if idx == 133:
                    point_REL = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 0, 255), thickness=-1)
                if idx == 159:
                    point_RET = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 0, 255), thickness=-1)
                if idx == 362:
                    point_LER = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 0, 255), thickness=-1)
                if idx == 374:
                    point_LEB = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 0, 255), thickness=-1)
                if idx == 263:
                    point_LEL = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 0, 255), thickness=-1)
                if idx == 386:
                    point_LET = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 0, 255), thickness=-1)
                if idx == 468:
                    point_REIC = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(255, 255, 0), thickness=-1)                    
                if idx == 469:
                    point_469 = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 255, 0), thickness=-1)
                if idx == 470:
                    point_470 = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 255, 0), thickness=-1)
                if idx == 471:
                    point_471 = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 255, 0), thickness=-1)
                if idx == 472:
                    point_472 = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 255, 0), thickness=-1)
                if idx == 473:
                    point_LEIC = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(0, 255, 255), thickness=-1)
                if idx == 474:
                    point_474 = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(255, 0, 0), thickness=-1)
                if idx == 475:
                    point_475 = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(255, 0, 0), thickness=-1)
                if idx == 476:
                    point_476 = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(255, 0, 0), thickness=-1)
                if idx == 477:
                    point_477 = (lm.x * img_w, lm.y * img_h)
                    #cv2.circle(image, (int(lm.x * img_w), int(lm.y * img_h)), radius=5, color=(255, 0, 0), thickness=-1)
                if idx == 33 or idx == 263 or idx == 1 or idx == 61 or idx == 291 or idx == 199:
                    if idx == 1:
                        nose_2d = (lm.x * img_w, lm.y * img_h)
                        nose_3d = (lm.x * img_w, lm.y * img_h, lm.z * 3000)
                    x, y = int(lm.x * img_w), int(lm.y * img_h)
                    face_2d.append([x,y])
                    face_3d.append([x,y,lm.z])
                #LEFT_IRIS = [473, 474, 475, 476, 477]
                if idx == 473 or idx == 362 or idx == 374 or idx == 263 or idx == 386: # iris points
                #if idx == 473 or idx == 474 or idx == 475 or idx == 476 or idx == 477: # eye border
                    if idx == 473:
                        left_pupil_2d = (lm.x * img_w, lm.y * img_h)
                        left_pupil_3d = (lm.x * img_w, lm.y * img_h, lm.z * 3000)
                    x, y = int(lm.x * img_w), int(lm.y * img_h)
                    left_eye_2d.append([x,y])
                    left_eye_3d.append([x,y,lm.z])
                    
                #RIGHT_IRIS = [468, 469, 470, 471, 472]
                if idx == 468 or idx == 33 or idx == 145 or idx == 133 or idx == 159: # iris points
                # if idx == 468 or idx == 469 or idx == 470 or idx == 471 or idx == 472: # eye border
                    if idx == 468:
                        right_pupil_2d = (lm.x * img_w, lm.y * img_h)
                        right_pupil_3d = (lm.x * img_w, lm.y * img_h, lm.z * 3000)
                    x, y = int(lm.x * img_w), int(lm.y * img_h)
                    right_eye_2d.append([x,y])
                    right_eye_3d.append([x,y,lm.z])
                if idx in right_eye_idx:
                    if idx == 33:
                        x1_r = lm.x
                    if idx == 133:
                        x4_r = lm.x
                    if idx == 144:
                        y6_r = lm.y
                    if idx == 153:
                        y5_r = lm.y
                    if idx == 158:
                        y3_r = lm.y
                    if idx == 160:
                        y2_r = lm.y
                if idx in left_eye_idx:
                    if idx == 263:
                        x1_l = lm.x
                    if idx == 362:
                        x4_l = lm.x
                    if idx == 373:
                        y6_l = lm.y
                    if idx == 380:
                        y5_l = lm.y
                    if idx == 385:
                        y3_l = lm.y
                    if idx == 387:
                        y2_l = lm.y
            
            # 4.4. - Draw the positions on the frame
            l_eye_width = point_LEL[0] - point_LER[0]
            l_eye_height = point_LEB[1] - point_LET[1]
            l_eye_center = [(point_LEL[0] + point_LER[0])/2 ,(point_LEB[1] + point_LET[1])/2]
            l_eye_area = (math.sqrt((point_LEL[0]-point_LER[0])**2+(point_LEL[1]-point_LER[1])**2)*math.sqrt((point_LEB[0]-point_LET[0])**2+(point_LEB[1]-point_LET[1])**2))/2
            #cv2.circle(image, (int(l_eye_center[0]), int(l_eye_center[1])), radius=int(horizontal_threshold * l_eye_width), color=(255, 0, 0), thickness=-1) #center of eye and its radius 
            #cv2.circle(image, (int(point_LEIC[0]), int(point_LEIC[1])), radius=3, color=(0, 255, 0), thickness=3) # Center of iris
            #cv2.circle(image, (int(l_eye_center[0]), int(l_eye_center[1])), radius=2, color=(128, 128, 128), thickness=3) # Center of eye

            r_eye_width = point_REL[0] - point_RER[0]
            r_eye_height = point_REB[1] - point_RET[1]
            r_eye_center = [(point_REL[0] + point_RER[0])/2 ,(point_REB[1] + point_RET[1])/2]
            r_eye_area = (math.sqrt((point_REL[0]-point_RER[0])**2+(point_REL[1]-point_RER[1])**2)*math.sqrt((point_REB[0]-point_RET[0])**2+(point_REB[1]-point_RET[1])**2))/2
            #cv2.circle(image, (int(r_eye_center[0]), int(r_eye_center[1])), radius=int(horizontal_threshold * r_eye_width), color=(255, 0, 0), thickness=-1) #center of eye and its radius 

            #cv2.circle(image, (int(point_REIC[0]), int(point_REIC[1])), radius=3, color=(0, 0, 255), thickness=-1) # Center of iris

            #cv2.circle(image, (int(r_eye_center[0]), int(r_eye_center[1])), radius=2, color=(128, 128, 128), thickness=-1) # Center of eye

            # speed reduction (comment out for full speed)
            time.sleep(1/25) # [s]
        
        face_2d = np.array(face_2d,dtype = np.float64)
        face_3d = np.array(face_3d,dtype = np.float64)
        left_eye_2d = np.array(left_eye_2d,dtype = np.float64)
        left_eye_3d = np.array(left_eye_3d,dtype = np.float64)
        right_eye_2d = np.array(right_eye_2d, dtype = np.float64)
        right_eye_3d = np.array(right_eye_3d, dtype = np.float64)
        focal_length = 1*img_w
        cam_matrix = np.array([[focal_length,0,img_h/2],
        [0,focal_length,img_w/2],
        [0,0,1]])
        dist_matrix = np.zeros((4,1),dtype = np.float64)
        success_face, rot_vec,trans_vec = cv2.solvePnP(face_3d,face_2d,cam_matrix,dist_matrix)
        success_left_eye, rot_vec_left_eye, trans_vec_left_eye = cv2.solvePnP(left_eye_3d,left_eye_2d,cam_matrix,dist_matrix)
        success_right_eye,rot_vec_right_eye,trans_vec_right_eye = cv2.solvePnP(right_eye_3d, right_eye_2d,cam_matrix,dist_matrix)

        rmat, jac = cv2.Rodrigues(rot_vec)
        rmat_left_eye, jac_left_eye = cv2.Rodrigues(rot_vec_left_eye)
        rmat_right_eye, jac_right_eye = cv2.Rodrigues(rot_vec_right_eye)

        angles, mtxR, mtxQ, Qx, Qy, Qz = cv2.RQDecomp3x3(rmat)
        angles_left_eye,mtxR_left_eye,mtxQ_left_eye,Qx_left_eye,Qy_left_eye,Qz_left_eye = cv2.RQDecomp3x3(rmat_left_eye)
        angles_right_eye,mtxR_right_eye,mtxQ_right_eye,Qx_right_eye,Qy_right_eye,Qz_right_eye = cv2.RQDecomp3x3(rmat_right_eye)

        pitch = angles[0] * 1800
        yaw = -angles[1] * 1800
        roll = 180 + (np.arctan2(point_RER[1]-point_LEL[1],point_RER[0]-point_LEL[0])*180/np.pi)
        if roll > 180:
            roll = roll - 360
        pitch_eye_left = angles_left_eye[0] * 1800
        yaw_eye_left = angles_left_eye[1] * 1800
        pitch_eye_right = angles_right_eye[0] * 1800
        yaw_eye_right = angles_right_eye[1] * 1800

        p1 = (int(nose_2d[0]),int(nose_2d[1]))
        p2 = (int(nose_2d[0] - yaw * 2),int(nose_2d[1]-pitch*10))
        #cv2.line(image,p1,p2,(255,0,0),3)

        is_3d=False
        if time.time()-start_setup > 4:
            if setup:
                if len(r_eye_height_avg_np) <= 0:
                    r_eye_ratio = 3
                    l_eye_ratio = 3
                    r_eye_height_avg =  -1
                    r_eye_width_avg = -1
                    l_eye_height_avg = -1
                    l_eye_width_avg = -1  
                else:
                    r_eye_ratio = np.mean(r_eye_width_avg_np)/np.mean(r_eye_height_avg_np)
                    l_eye_ratio =  np.mean(l_eye_width_avg_np)/ np.mean(l_eye_height_avg_np)
                    r_eye_height_avg =  np.mean(r_eye_height_avg_np)
                    r_eye_width_avg = np.mean(r_eye_width_avg_np)
                    l_eye_height_avg = np.mean(l_eye_height_avg_np)
                    l_eye_width_avg = np.mean(l_eye_width_avg_np)
                s_distance = np.mean(fixed_size)
                setup = False
            
            if is_3d:
                left_eye_3d_projection, left_eye_jacobian = cv2.projectPoints(left_eye_3d, rot_vec_left_eye, trans_vec_left_eye, cam_matrix, dist_matrix)
                p1_l = (int(left_pupil_2d[0]),int(left_pupil_2d[1]))
                p2_l = (int(left_pupil_2d[0] + yaw_eye_left * 2),int(left_pupil_2d[1]-pitch_eye_left*10))
                #cv2.line(image,p1_l,p2_l,(255,0,0),3)
                right_eye_3d_projection, right_eye_jacobian = cv2.projectPoints(right_eye_3d, rot_vec_right_eye, trans_vec_right_eye, cam_matrix, dist_matrix)
                p1_r = (int(right_pupil_2d[0]),int(right_pupil_2d[1]))
                p2_r = (int(right_pupil_2d[0] + yaw_eye_right * 2),int(right_pupil_2d[1]-pitch_eye_right*10))
                #cv2.line(image,p1_r,p2_r,(255,0,0),3)
            
            else:
                
                if l_eye_area<40 and r_eye_area<40:
                    cv2.putText(image, f'Warning: the eyes are too small', (0,60), cv2.FONT_HERSHEY_SIMPLEX, 1 , (0, 255, 255), 3)
                    cv2.putText(image, f'you are too far', (150,90), cv2.FONT_HERSHEY_SIMPLEX, 1 , (0, 255, 255), 3)

                    yaw_eyes=0
                    pitch_eyes=0
                else:

                    if r_eye_area>40:                        
                        r_eye_center_S = [(point_REL[0] + point_RER[0])/2,(point_SBR[1]+point_STR[1])/2]
                        if r_eye_width_avg==-1:
                            yaw_right=((point_REIC[0]-r_eye_center[0])*105)/(r_eye_width/4)
                            pitch_right=((point_REIC[1]-r_eye_center_S[1])*75)/(r_eye_height/2)
                        else:
                            yaw_right=((point_REIC[0]-r_eye_center[0])*105)/(r_eye_width_avg/4)
                            pitch_right=((point_REIC[1]-r_eye_center_S[1])*75)/(r_eye_height_avg/2)
                        p1_r=(int(r_eye_center_S[0]),int(r_eye_center[1]))
                        reic=int(point_REIC[0]),int(point_REIC[1])
                        yaw_eyes=-yaw_right
                        pitch_eyes=-pitch_right
                        #reic_end_point = (int(point_REIC[0]+yaw_right*2),int(point_REIC[1]+pitch_right*2))
                        #cv2.line(image,reic,reic_end_point,(255,0,0),3)

                    else:
                        cv2.putText(image, f'Warning: the right eye is too small', (0,60), cv2.FONT_HERSHEY_SIMPLEX, 1 , (0, 255, 255), 3)


                    if l_eye_area>40:                
                        l_eye_center_S = [(point_LEL[0] + point_LER[0])/2,(point_SBL[1]+point_STL[1])/2]
                
                        if l_eye_width_avg==-1:
                            yaw_left=((point_LEIC[0]-l_eye_center[0])*105)/(l_eye_width/4)
                            pitch_left=((point_LEIC[1]-l_eye_center_S[1])*75)/(l_eye_height/2)
                        else:
                            yaw_left=((point_LEIC[0]-l_eye_center[0])*105)/(l_eye_width_avg/4)
                            pitch_left=((point_LEIC[1]-l_eye_center_S[1])*75)/(l_eye_height_avg/2)
                
                        p1_l=(int(l_eye_center_S[0]),int(l_eye_center[1]))
                        leic=int(point_LEIC[0]),int(point_LEIC[1])

                        if yaw_eyes<0:
                            yaw_eyes=(yaw_eyes-yaw_left)/2

                        if pitch_eyes<0:
                            pitch_eyes=(pitch_eyes-pitch_left)/2

                        #leic_end_point = (int(point_LEIC[0]+yaw_left*2),int(point_LEIC[1]+pitch_left*2))
                        #cv2.line(image,leic,leic_end_point,(255,0,0),3)

                    else:
                        cv2.putText(image, f'Warning: the left eye is too small', (0,60), cv2.FONT_HERSHEY_SIMPLEX, 1 , (0, 255, 255), 3)

            end = time.time()

            totalTime = end-start
            
            
            EAR_l = (l_eye_ratio)*(np.abs(y2_l-y6_l)+np.abs(y3_l-y5_l))/(2*np.abs(x1_l-x4_l))
            EAR_r = (r_eye_ratio)*(np.abs(y2_r-y6_r)+np.abs(y3_r-y5_r))/(2*np.abs(x1_r-x4_r))
            EAR = 100*(EAR_r+EAR_l)/2

            if EAR >= 80:
                tempo += totalTime
            else:
                tempo= 0
            
            if tempo >= 10:
                cv2.putText(image, f'You are asleep', (0,150), cv2.FONT_HERSHEY_SIMPLEX, 2 , (0, 0, 255), 3)

            total_yaw = yaw+yaw_eyes
            total_pitch = pitch+pitch_eyes
            print('Total pitch:',total_pitch,'face pitch:',pitch,'pitch eyes:',pitch_eyes)
            print('Total Yaw:',total_yaw,'face yaw:',yaw,'yaw eyes:',yaw_eyes)
            if abs(roll) > 30 or abs(total_yaw) > 30 or abs(total_pitch) > 30:
                cv2.putText(image, f'You are distracted', (0,300), cv2.FONT_HERSHEY_SIMPLEX, 2 , (0, 0, 255), 3)
                0 == 0

        else:
            print(r_eye_width/r_eye_height,l_eye_width/l_eye_height)
            if r_eye_width/r_eye_height > 2.5 and r_eye_width/r_eye_height < 3.5 and l_eye_width/l_eye_height > 2.5 and l_eye_width/l_eye_height < 3.5:
                r_eye_width_avg_np.append(r_eye_width)
                r_eye_height_avg_np.append(r_eye_height)
                l_eye_width_avg_np.append(l_eye_width)
                l_eye_height_avg_np.append(l_eye_height)
            fixed_size.append(abs(point_52[1] - point_STR[1]))
            end = time.time()
            totalTime = end-start
        #print("FPS:", fps)
        if totalTime>0:
            
            fps = 1 / totalTime

        else:

            fps=0

        # 4.5 - Show the frame to the user

        cv2.imshow('Technologies for Autonomous Vehicles - Driver Monitoring Systems using AI code sample', image)       

    if cv2.waitKey(5) & 0xFF == 27:

        break

# 5 - Close properly soruce and eventual log file

cap.release()

#log_file.close()

# [EOF]